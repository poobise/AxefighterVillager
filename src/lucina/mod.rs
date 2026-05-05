mod acmd;
//mod frame;


pub fn install() {
    let agent = &mut smashline::Agent::new("lucina");
    acmd::install(agent);

    agent.install();
}
