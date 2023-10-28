use crate::{v, F64Key, V};

/// Cluster the given points into `k` clusters using the [K-Means Algorithm][0].
///
/// The algorithm will run for at most `max_iter` iterations and it is not
/// guaranteed to converge. However, for our use case it is good enough and also
/// reasonably fast.
///
/// [0]: https://en.wikipedia.org/wiki/K-means_clustering
pub fn kmeans(pts: &[V], k: usize, max_iter: usize) -> Vec<(V, Vec<usize>)> {
    let mut centroids: Vec<_> = pts.iter().take(k).map(|v| (*v, vec![])).collect();
    let mut new_centroids = vec![(v(0, 0), vec![]); centroids.len()];

    for _ in 0..max_iter {
        for (i, p) in pts.iter().enumerate() {
            let ((nc, idxs), _) = new_centroids
                .iter_mut()
                .zip(&centroids)
                .min_by_key(|(_, (c, _))| F64Key(c.dist2(*p)))
                .unwrap();

            *nc += *p;
            idxs.push(i);
        }

        let mut unchanged = true;
        for ((nc, idxs), (oc, oidxs)) in new_centroids.iter_mut().zip(&mut centroids) {
            debug_assert!(!idxs.is_empty());

            *nc /= idxs.len() as f64;

            if !nc.almost_equal(*oc) {
                unchanged = false;
            }

            oidxs.clear();
            *oc = v(0, 0);
        }

        std::mem::swap(&mut centroids, &mut new_centroids);

        if unchanged {
            return centroids;
        }
    }

    centroids
}
