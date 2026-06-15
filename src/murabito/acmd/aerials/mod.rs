mod attackairn;
mod attackairf;
mod attackairb;
mod attackairhi;
mod attackairlw;

pub fn install() {
    attackairn::install();
    attackairf::install();
    attackairb::install();
    attackairhi::install();
    attackairlw::install();
}