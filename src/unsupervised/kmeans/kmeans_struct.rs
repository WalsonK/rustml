#[repr(C)]
pub struct KMeans {
    pub cluster_centroid: Vec<Vec<f32>>,
    pub labels: Vec<usize>,
    pub clusters_std: Vec<Vec<f32>>,
}