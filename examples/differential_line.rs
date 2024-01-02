use lart::*;

sketch_parms! {
    page: Page = Page::A4,
    t: usize = 50,
    influence_radius: f64 = 10.0,
    shape: Choice = Choice::new("test", &["test", "circle"]),
    initial_points: u16 = 10,
    seeds: usize = 1,
    sampling_step: f64 = 5.0,
}

#[derive(Clone, Debug, PartialEq)]
struct Agent {
    pos: V,
    dir: V,
    speed: f64,
}

fn main() {
    let parms = Parms::from_cli();
    let mut doc = Sketch::new("differential_line").with_page(parms.page);

    let mut clusters: Vec<Vec<Agent>>;
    if parms.shape.value() == "test" {
        clusters = vec![[v(-1, -1), v(1, -1), v(1, 1), v(-1, 1)]
            .into_iter()
            .map(|d| Agent {
                pos: doc.page_bbox().center() + d * 30.0,
                dir: d.normalized(),
                speed: 1.0,
            })
            .collect()];
    } else {
        let mut packer = CirclePacker::new(doc.page_bbox());
        packer.min_radius = 10.0;
        packer.max_radius = doc.page_bbox().width().min(doc.page_bbox().height()) * 0.45;
        for _ in 0..100_000 {
            if packer.circles().len() >= parms.seeds {
                break;
            }
            packer.generate(&mut doc);
        }

        clusters = packer
            .circles()
            .iter()
            .map(|(c, r)| match parms.shape.value() {
                "circle" => Path::circle(*c, r * 0.85, parms.initial_points)
                    .iter()
                    .map(|p| Agent::with_rand_dir(&mut doc, p))
                    .collect(),
                s => panic!("unsupported shape {s}"),
            })
            .collect();
    }

    for _ in 0..parms.t {
        let mut agents_bbox = Rect::new(clusters[0][0].pos);

        clusters = Vec::from_iter(clusters.iter().map(|agents| {
            let mut new_agents = vec![];

            for i in 0..agents.len() {
                let a0 = agents[i].clone();
                let a1 = agents[(i + 1) % agents.len()].clone();

                agents_bbox.expand(a0.pos);

                let osz = new_agents.len();
                new_agents.extend(sample_seg(a0.pos, a1.pos, parms.sampling_step, false).map(
                    |s| Agent {
                        pos: s.point,
                        dir: linterp(a0.dir, a1.dir, s.t / s.segment_len),
                        speed: a0.speed,
                    },
                ));
                // if we inserted a point and it's too close to the next agent,
                // discard it because it causes overlaps
                if osz != new_agents.len()
                    && new_agents.last().unwrap().pos.dist(a1.pos) < parms.sampling_step
                {
                    new_agents.pop();
                }
            }

            new_agents
        }));

        let index = QuadTree::new(agents_bbox, clusters.iter().flatten().cloned().collect());

        clusters = Vec::from_iter(clusters.iter().map(|agents| {
            Vec::from_iter(agents.iter().map(|a| {
                let mut na = a.clone();

                let mut neighbors_dir = a.dir;
                let mut neighbors = 1.0;
                for aa in index.in_range(a.pos, parms.influence_radius) {
                    if a.pos == aa.pos {
                        continue;
                    }

                    let l = a.pos.dist(aa.pos);
                    neighbors += 1.0;
                    neighbors_dir += (aa.pos - a.pos) / l * (1.0 - l / parms.influence_radius);
                }

                neighbors_dir = (neighbors_dir / neighbors).normalized();

                na.dir = -neighbors_dir;
                na.pos += na.dir * na.speed;

                na
            }))
        }));
    }

    for agents in clusters {
        let p: Path = agents.into_iter().map(|a| a.pos).collect();
        doc.geometry(spline::sample(&p.closed(), 0.1));
    }

    doc.fit_to_page(20.0);
    doc.save().unwrap();
}

impl Agent {
    fn with_rand_dir(rng: &mut impl Rng, pos: V) -> Self {
        Agent {
            pos,
            dir: V::polar(rng.gen_range(0.0..=TAU), 1.0),
            speed: rng.gen_range(0.1..=1.0),
        }
    }
}

impl QuadTreeElem for Agent {
    fn reference(&self) -> V {
        self.pos
    }
}
