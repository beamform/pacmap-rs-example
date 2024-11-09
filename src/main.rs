//! PaCMAP dimensional reduction implemented in Rust
//!
//! This module provides a Rust implementation of the PaCMAP algorithm for
//! dimensionality reduction and visualization. It demonstrates using PaCMAP to
//! reduce the USPS digits dataset to 2D and visualize the results.

use anyhow::{Context, Result};
use mimalloc::MiMalloc;
use ndarray::{Array1, Array2, ArrayView2};
use ndarray_npy::ReadNpyExt;
use pacmap::Configuration;
use plotly::common::{Mode, Title};
use plotly::{Layout, Plot, Scatter};
use std::io::Cursor;
use std::time::Instant;
use tracing::info;

// Use the MiMalloc allocator globally for better performance
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// URLs for the USPS digits dataset and labels
const DATA_URL: &str = "https://raw.githubusercontent.com/YingfanWang/PaCMAP/master/data/USPS.npy";
const LABELS_URL: &str =
    "https://raw.githubusercontent.com/YingfanWang/PaCMAP/master/data/USPS_labels.npy";

/// Main entry point that downloads USPS data, runs PaCMAP dimensionality
/// reduction, and creates an interactive visualization
///
/// # Errors
/// Returns an error if:
/// - Data download fails
/// - Data parsing fails
/// - PaCMAP embedding fails
/// - Plot creation fails
fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Download and load data directly into memory
    info!("Downloading and loading data...");
    let x = download_and_load_array2(DATA_URL)?;

    // Flatten the array to (n_samples, n_features)
    let n_samples = x.shape()[0];
    let n_features: usize = x.shape()[1..].iter().product();
    let x = x.to_shape::<(usize, usize)>((n_samples, n_features))?;

    info!("Downloading and loading labels...");
    let labels = download_and_load_array1(LABELS_URL)?;

    // Configure PaCMAP with empirically optimal parameters
    let config = Configuration::builder()
        .embedding_dimensions(2)
        .override_neighbors(10)
        .mid_near_ratio(0.5)
        .far_pair_ratio(2.0)
        .build();

    // Run PaCMAP dimensionality reduction
    info!("Running PaCMAP on x with shape {:?}...", x.shape());
    let start = Instant::now();
    let (embedding, _) = pacmap::fit_transform(x.view(), config)?;
    let duration = Instant::now().duration_since(start);
    info!("PaCMAP completed in {} ms", duration.as_millis());

    // Create and save interactive visualization
    let scatter = create_scatter_plot(embedding.view(), &labels)?;

    let layout = Layout::new()
        .title(Title::with_text("PaCMAP Embedding"))
        .width(600)
        .height(600);

    info!("Saving visualization...");
    let mut plot = Plot::new();
    plot.add_trace(scatter);
    plot.set_layout(layout);
    plot.write_html("pacmap_visualization.html");

    info!("Done! Visualization saved to pacmap_visualization.html");
    Ok(())
}

/// Creates an interactive scatter plot from embedding coordinates and labels
///
/// # Arguments
/// * `embedding` - 2D array containing embedded coordinates
/// * `labels` - 1D array of integer labels for each point
///
/// # Errors
/// Returns error if scatter plot creation fails
fn create_scatter_plot(
    embedding: ArrayView2<f32>,
    labels: &Array1<i32>,
) -> Result<Box<Scatter<f32, f32>>> {
    let x = embedding.column(0).to_vec();
    let y = embedding.column(1).to_vec();

    // Create scatter plot with points colored by label
    let scatter = Scatter::new(x, y).mode(Mode::Markers).marker(
        plotly::common::Marker::new()
            .color_array(labels.to_vec())
            .show_scale(true)
            .size(2),
    );

    Ok(scatter)
}

/// Downloads and parses a 2D numpy array from a URL
///
/// # Arguments
/// * `url` - URL of the .npy file to download
///
/// # Errors
/// Returns error if:
/// - Download fails
/// - Byte parsing fails
/// - NPY parsing fails
fn download_and_load_array2(url: &str) -> Result<Array2<f32>> {
    let response =
        reqwest::blocking::get(url).with_context(|| format!("Failed to download from {}", url))?;

    let bytes = response
        .bytes()
        .with_context(|| format!("Failed to read bytes from {}", url))?;

    // Read NPY data from memory
    let reader = Cursor::new(&bytes[..]);
    let array = Array2::<f32>::read_npy(reader)
        .with_context(|| format!("Failed to parse NPY data from {}", url))?;

    Ok(array)
}

/// Downloads and parses a 1D numpy array from a URL
///
/// # Arguments
/// * `url` - URL of the .npy file to download
///
/// # Errors
/// Returns error if:
/// - Download fails
/// - Byte parsing fails
/// - NPY parsing fails
fn download_and_load_array1(url: &str) -> Result<Array1<i32>> {
    let response =
        reqwest::blocking::get(url).with_context(|| format!("Failed to download from {}", url))?;

    let bytes = response
        .bytes()
        .with_context(|| format!("Failed to read bytes from {}", url))?;

    // Read NPY data from memory
    let reader = Cursor::new(&bytes[..]);
    let array = Array1::<i32>::read_npy(reader)
        .with_context(|| format!("Failed to parse NPY data from {}", url))?;

    Ok(array)
}
