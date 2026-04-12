mod attack3s; //ftilt
//mod attackhi3;
//mod attacklw3;

pub fn install(agent: &mut smashline::Agent) {
    attack3s::install(agent);
    //attackairhi::install(agent);
    //attackairlw::install(agent);
}