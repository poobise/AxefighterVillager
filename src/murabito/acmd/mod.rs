mod tilts;
mod smashes;
mod aerials;
mod attackdash;

pub fn install(agent: &mut smashline::Agent) {
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    attackdash::install(agent);

}