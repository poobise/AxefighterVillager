mod acmd;
//mod frame;
//mod bullet;

pub fn install() {
    let agent = &mut smashline::Agent::new("murabito");
    acmd::install(agent);
    //bullet::install();

    agent.install();
}
