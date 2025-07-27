use super::kmeans_struct::KMeans;

#[no_mangle]
pub(crate) extern "C" fn init() -> Box<KMeans> {
    let model = Box::new(KMeans {
        cluster_centroid: Vec::new(),
        labels: Vec::new(),
        clusters_std: Vec::new()
    });

    model
}