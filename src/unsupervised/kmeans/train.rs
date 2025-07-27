use rand::Rng;
use crate::basic_function::tools::euclidean_distance;
use super::kmeans_struct::KMeans;

#[no_mangle]
pub(crate) extern "C" fn train(
    model: &mut KMeans, nb_cluster: i32, max_iter: i32,
    datas: *const f32, datas_len: i32, n_features: usize
) {
    // Convert datas
    let data_slice = unsafe { std::slice::from_raw_parts(datas, datas_len as usize) };
    let data: Vec<Vec<f32>> = data_slice.chunks(n_features).map(|x| x.to_vec()).collect();

    model.cluster_centroid = initialize_cluster(nb_cluster, n_features, 0);
    model.labels = vec![0; data.len()];
    model.clusters_std = vec![vec![0.0; n_features]; nb_cluster as usize];

    for _ in 0..max_iter {
        let labels = assign_clusters(&data, &model.cluster_centroid);
        let new_centroids = update_centroids(&data, &labels, nb_cluster as usize, n_features);

        if has_converged(&model.cluster_centroid, &new_centroids, 1e-4) {
            model.labels = labels;
            model.cluster_centroid = new_centroids;
            break;
        }

        model.labels = labels;
        model.cluster_centroid = new_centroids;
    }

    // TODO: Calcul des std par cluster ici
}

fn initialize_cluster(nb_cluster: i32, n_features: usize, mode: i32) -> Vec<Vec<f32>> {
    let mut cluster = Vec::new();

    // Full Random
    if mode == 0 {
        let mut rng = rand::rng();
        for _ in 0..nb_cluster {
            let point: Vec<f32> = (0..n_features).map(|_| rng.gen()).collect();
            cluster.push(point);
        }
    }

    // Based on Random data point
    if mode == 1 {}

    cluster
}

fn assign_clusters(data: &Vec<Vec<f32>>, centroids: &Vec<Vec<f32>>) -> Vec<usize> {
    data.iter()
        .map(|point| {
            centroids
                .iter()
                .enumerate()
                .map(|(i, centroid)| (i, euclidean_distance(point, centroid)))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap()
                .0
        })
        .collect()
}

fn update_centroids(data: &Vec<Vec<f32>>, labels: &Vec<usize>, nb_cluster: usize, n_features: usize) -> Vec<Vec<f32>> {
    let mut centroids = vec![vec![0.0; n_features]; nb_cluster];
    let mut counts = vec![0; nb_cluster];

    for (point, &label) in data.iter().zip(labels.iter()) {
        for i in 0..n_features {
            centroids[label][i] += point[i];
        }
        counts[label] += 1;
    }

    for k in 0..nb_cluster {
        if counts[k] > 0 {
            for i in 0..n_features {
                centroids[k][i] /= counts[k] as f32;
            }
        }
    }

    centroids
}

fn has_converged(old: &Vec<Vec<f32>>, new: &Vec<Vec<f32>>, tol: f32) -> bool {
    old.iter()
        .zip(new.iter())
        .all(|(a, b)| a.iter().zip(b.iter()).all(|(x, y)| (x - y).abs() < tol))
}
