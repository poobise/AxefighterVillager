mod acmd;
//mod frame;


pub fn install() {
    let agent = &mut smashline::Agent::new("murabito")
    .set_costume(get_marked_costumes("murabito","murabito"));
    acmd::install(agent);

    agent.install();
}
