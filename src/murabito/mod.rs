#![allow(static_mut_refs, unused_mut)]

mod acmd;
//mod frame;
//mod bullet;

pub fn install() {
    //let agent = &mut smashline::Agent::new("murabito");
    acmd::install();
    //bullet::install();

    //agent.install();
}
