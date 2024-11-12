pub mod bitmap_layer;
pub mod ai_synchronizer;
pub mod parcel_state;
pub mod agent;

pub fn init_bitmap_layer() {
    println!("AI-driven Bitmap layer initialized.");
    bitmap_layer::initialize_parcels();
}

