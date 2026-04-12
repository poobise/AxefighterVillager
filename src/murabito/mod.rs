mod acmd;
//mod frame;


pub fn install() {
    let agent = &mut smashline::Agent::new("murabito");
    acmd::install(agent);

    agent.install();
}
