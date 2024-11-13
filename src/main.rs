//! PaCMAP dimensionality reduction example
//!
//! This example demonstrates using the PaCMAP algorithm to reduce the MNIST digits dataset
//! from 784 dimensions to 2 dimensions for visualization. It loads the MNIST data, applies
//! PaCMAP reduction, and creates an interactive scatter plot colored by digit class.
//!
//! The example showcases:
//! - Loading and preprocessing MNIST data
//! - Configuring and running PaCMAP dimensionality reduction
//! - Creating interactive visualizations with plotly

use anyhow::{Context, Result};
use mimalloc::MiMalloc;
use mnist::{Mnist, MnistBuilder};
use ndarray::{Array1, Array3, ArrayView2};
use pacmap::Configuration;
use plotly::common::{ColorScale, ColorScalePalette, Marker, Mode, Title};
use plotly::{Layout, Plot, Scatter};
use std::time::Instant;
use tracing::info;
use ColorScale::Palette;

// Use MiMalloc globally for improved memory allocation performance
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Run PaCMAP dimensionality reduction on MNIST and create visualization
///
/// Loads the MNIST dataset, applies PaCMAP to reduce dimensionality to 2D,
/// and creates an interactive scatter plot visualization colored by digit class.
///
/// # Errors
/// Returns an error if:
/// - MNIST data loading fails
/// - Array reshaping operations fail
/// - PaCMAP embedding fails
/// - Plot creation fails
fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load and combine training and test MNIST data
    info!("Loading MNIST dataset...");
    let Mnist {
        mut trn_img,
        mut trn_lbl,
        mut tst_img,
        mut tst_lbl,
        ..
    } = MnistBuilder::new()
        .base_url("https://ossci-datasets.s3.amazonaws.com/mnist/")
        .label_format_digit()
        .download_and_extract()
        .training_set_length(60_000)
        .test_set_length(10_000)
        .finalize();

    trn_img.append(&mut tst_img);

    // Normalize pixel values to [0,1] and reshape to (n_samples, n_features)
    let x = Array3::from_shape_vec((70_000, 28, 28), trn_img)
        .context("Error converting images to Array3")?
        .map(|x| *x as f32 / 255.0);

    // Reshape to (n_samples, n_features)
    let x = x.into_shape_with_order((70_000, 784))?;

    trn_lbl.append(&mut tst_lbl);

    // Convert labels to Array1
    let labels = Array1::from_vec(trn_lbl).mapv(|x| x as i32);

    // Configure PaCMAP with empirically optimal parameters for MNIST
    let config = Configuration::builder()
        .embedding_dimensions(2)
        .override_neighbors(10)
        .mid_near_ratio(0.5)
        .far_pair_ratio(2.0)
        .build();

    // Run PaCMAP reduction and time it
    info!("Running PaCMAP on MNIST with shape {:?}...", x.shape());
    let start = Instant::now();
    let (embedding, _) = pacmap::fit_transform(x.view(), config)?;
    let duration = Instant::now().duration_since(start);
    info!("PaCMAP completed in {} ms", duration.as_millis());

    // Create and save interactive visualization
    let scatter = create_scatter_plot(embedding.view(), &labels)?;

    let layout = Layout::new()
        .title(Title::with_text("PaCMAP Embedding of MNIST"))
        .width(800)
        .height(800);

    info!("Saving visualization...");
    let mut plot = Plot::new();
    plot.add_trace(scatter);
    plot.set_layout(layout);
    plot.write_html("pacmap_visualization.html");

    info!("Done! Visualization saved to pacmap_visualization.html");
    Ok(())
}

/// Creates an interactive scatter plot of the embedding coordinates
///
/// Creates a plotly scatter plot with points colored by their digit class,
/// using the Portland color palette for visual distinction between classes.
///
/// # Arguments
/// * `embedding` - 2D array of shape (n_samples, 2) containing embedded coordinates
/// * `labels` - 1D array of shape (n_samples,) containing digit labels (0-9)
///
/// # Errors
/// Returns an error if scatter plot creation fails
fn create_scatter_plot(
    embedding: ArrayView2<f32>,
    labels: &Array1<i32>,
) -> Result<Box<Scatter<f32, f32>>> {
    let x = embedding.column(0).to_vec();
    let y = embedding.column(1).to_vec();

    // Create scatter with digit class coloring
    let scatter = Scatter::new(x, y).mode(Mode::Markers).marker(
        Marker::new()
            .color_array(labels.to_vec())
            .show_scale(true)
            .color_scale(Palette(ColorScalePalette::Portland))
            .size(2),
    );

    Ok(scatter)
}
