use std::{
    env::{self, current_dir},
    io,
    path::Path,
    process::Command,
    sync::{
        mpsc::{sync_channel, RecvTimeoutError, SyncSender},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

mod rpc;
mod schema;
mod sketch;

use schema::Manifest;
use sketch::*;

use vsvg::exports::egui::{self, mutex::Mutex, Context, DragValue};
use vsvg_viewer::{
    exports::eframe::CreationContext, show_with_viewer_app, DocumentWidget, ViewerApp,
};

const SLOW_TIMEOUT: Duration = Duration::from_millis(5000);
const FAST_TIMEOUT: Duration = Duration::from_millis(1500);

pub struct App {
    sketch_input: Arc<Mutex<SketchInput>>,
    sketch_output: Arc<Mutex<SketchOutput>>,
    worker: Option<(JoinHandle<()>, SyncSender<()>)>,
}

fn main() -> anyhow::Result<()> {
    show_with_viewer_app(App::new())
}

impl App {
    pub fn new() -> Self {
        Self {
            sketch_input: Arc::new(Mutex::new(SketchInput {
                timeout: SLOW_TIMEOUT,
                cmd: env::args().skip(1).collect(),
                parameters: Manifest::new(),
            })),
            sketch_output: Arc::new(Mutex::new(SketchOutput::empty())),
            worker: None,
        }
    }
}

impl ViewerApp for App {
    fn title(&self) -> String {
        "lart viewer".to_owned()
    }

    fn setup(
        &mut self,
        cc: &CreationContext,
        _document_widget: &mut DocumentWidget,
    ) -> anyhow::Result<()> {
        let sketch_output = Arc::clone(&self.sketch_output);
        let sketch_input = Arc::clone(&self.sketch_input);
        let ctx = cc.egui_ctx.clone();

        let (sx, rx) = sync_channel(1);
        let worker = thread::spawn(move || loop {
            let inp = sketch_input.lock().clone();

            if let Ok(res) = sketch_run(&inp) {
                {
                    let mut inp = sketch_input.lock();
                    let mut old_parm = std::mem::take(&mut inp.parameters).into_iter().peekable();

                    for (parm, schema) in &res.manifest {
                        let mut schema = schema.clone();

                        if let Some((oparm, oschema)) = old_parm.peek() {
                            if oparm == parm {
                                schema.take_value_from(oschema);
                                old_parm.next();
                            }
                        }

                        inp.parameters.push((parm.clone(), schema));
                    }
                }

                *sketch_output.lock() = res;

                ctx.request_repaint();
            }

            match rx.recv_timeout(inp.timeout) {
                Ok(_) | Err(RecvTimeoutError::Disconnected) => break,
                _ => {}
            }
        });

        self.worker = Some((worker, sx));
        Ok(())
    }

    fn on_exit(&mut self) {
        if let Some(worker) = self.worker.take() {
            worker.1.send(()).unwrap();
            worker.0.join().unwrap();
        }
    }

    fn handle_input(&mut self, ctx: &Context, _document_widget: &mut DocumentWidget) {
        ctx.input_mut(|i| {
            if i.consume_key(egui::Modifiers::NONE, egui::Key::Space) {
                let src = self.sketch_output.lock().svg_filepath.clone();

                if let Some(src) = src {
                    if let Err(e) = like_sketch(&src) {
                        eprint!("Cannot optimize sketch {e}");
                    }
                }
            }

            if i.consume_key(egui::Modifiers::NONE, egui::Key::F) {
                self.sketch_input.lock().timeout = FAST_TIMEOUT;
            } else if i.consume_key(egui::Modifiers::SHIFT, egui::Key::F) {
                self.sketch_input.lock().timeout = SLOW_TIMEOUT;
            }
        });
    }

    fn show_central_panel(
        &mut self,
        _ui: &mut egui::Ui,
        document_widget: &mut DocumentWidget,
    ) -> anyhow::Result<()> {
        if let Some(svg) = self.sketch_output.lock().svg.take() {
            document_widget.set_document(Arc::new(svg));
        }

        Ok(())
    }

    fn show_panels(
        &mut self,
        ctx: &egui::Context,
        _document_widget: &mut DocumentWidget,
    ) -> anyhow::Result<()> {
        let mut inp = self.sketch_input.lock();

        egui::SidePanel::right("parms_panel").show(ctx, |ui| {
            // TODO: if something changed re-run sketch

            for (parm, schema) in inp.parameters.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(parm.clone());

                    match schema {
                        schema::Parm::String { value } => {
                            ui.text_edit_singleline(value);
                        }
                        schema::Parm::Bool { value } => {
                            ui.checkbox(value, "");
                        }
                        schema::Parm::Int { value, min, max } => {
                            ui.add(DragValue::new(value).clamp_range(*min..=*max).speed(1));
                        }
                        schema::Parm::UInt { value, min, max } => {
                            ui.add(DragValue::new(value).clamp_range(*min..=*max).speed(1));
                        }
                        schema::Parm::Double { value, min, max } => {
                            ui.add(
                                DragValue::new(value)
                                    .clamp_range(*min..=*max)
                                    .speed(0.1)
                                    .max_decimals(3),
                            );
                        }
                        schema::Parm::Choice { value, choices } => {
                            for c in choices {
                                ui.selectable_value(value, c.clone(), c.clone());
                            }
                        }
                    }
                });
            }
        });

        Ok(())
    }
}

fn like_sketch(src: &str) -> io::Result<()> {
    let outdir = current_dir().unwrap().join("liked");
    let out = outdir.join(Path::new(src).file_name().unwrap());

    // TODO: this should be done with vsvg once it supports simplifying and
    // merging (it already supports sorting!)
    Command::new("vpype")
        .args([
            "read",
            &src,
            "linesimplify",
            "linemerge",
            "linesort",
            "write",
            out.to_str().unwrap(),
        ])
        .spawn()?;

    Ok(())
}
