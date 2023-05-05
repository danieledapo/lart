use crate::{Geometry, Polygon, Rect, V};

/// Find a [triangulation][0] of the given set of points.
///
/// [0]: https://en.wikipedia.org/wiki/Triangulation
pub fn triangulate(pts: &[V]) -> Geometry {
    let pts = pts
        .as_ref()
        .iter()
        .map(|vv| delaunator::Point { x: vv.x, y: vv.y })
        .collect::<Vec<_>>();
    let tri = delaunator::triangulate(&pts);

    let polys = tri
        .triangles
        .chunks_exact(3)
        .map(|vs| {
            Polygon::from([
                (pts[vs[0]].x, pts[vs[0]].y),
                (pts[vs[1]].x, pts[vs[1]].y),
                (pts[vs[2]].x, pts[vs[2]].y),
            ])
        })
        .collect();

    Geometry::from_polygons(polys)
}

/// Generate the [Voronoi diagram][0] for the given set of points clipping it to
/// the given rect.
///
/// The clipping rect is necessary because the Voronoi diagram extends to
/// infinity geometrically speaking, but that's likely not what we want.
///
/// [0]: https://en.wikipedia.org/wiki/Voronoi_diagram
pub fn voronoi(pts: &[V], clip: &Rect) -> Geometry {
    let pts = pts
        .as_ref()
        .iter()
        .map(|vv| voronoice::Point { x: vv.x, y: vv.y })
        .collect::<Vec<_>>();

    let c = clip.center();
    voronoice::VoronoiBuilder::default()
        .set_sites(pts)
        .set_bounding_box(voronoice::BoundingBox::new(
            voronoice::Point { x: c.x, y: c.y },
            clip.width(),
            clip.height(),
        ))
        .build()
        .map_or_else(Geometry::new, |vor| {
            Geometry::from_polygons(
                vor.iter_cells()
                    .map(|p| Polygon::from(p.iter_vertices().map(|p| V::new(p.x, p.y))))
                    .collect::<Vec<_>>(),
            )
        })
}
