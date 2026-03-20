use maud::{html, Markup, DOCTYPE};
use crate::domain::Votingmachine;
use crate::interfaces::lexicon::Lexicon;
use super::WebRoutes;

pub fn vote_form(routes: &WebRoutes, lexicon: &Lexicon) -> Markup {
    html! {
        h2 { "Urne" }
        form hx-post=(routes.vote) hx-target="#outcome" hx-swap="innerHTML" {
            label {
                (lexicon.voter)
                input type="text" name="voter";
            }
            br;
            label {
                (lexicon.candidate)
                input type="text" name="candidat";
            }
            br;
            input type="submit" value="voter";
        }
        p id="outcome" {}
    }
}

pub fn voting_machine(_routes: &WebRoutes, lexicon: &Lexicon, machine: &Votingmachine) -> Markup {
    let scoreboard = machine.get_scoreboard();
    let voters_who_voted = machine.get_voters_who_voted();
    html! {
        h2 { "Scores" }
        table {
            @for (candidate, score) in &scoreboard.scores {
                tr {
                    td { (candidate.0) }
                    td { (score.0) }
                }
            }
            tr {
                td { (lexicon.blank) }
                td { (scoreboard.blank_score.0) }
            }
            tr {
                td { (lexicon.invalid) }
                td { (scoreboard.invalid_score.0) }
            }
        }
        h2 { "Votants" }
        ul {
            @for voter in voters_who_voted {
                li { (voter.0) }
            }
        }
    }
}

pub fn index(routes: &WebRoutes, lexicon: &Lexicon, machine: &Votingmachine) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "Machine de vote" }
                script src="https://unpkg.com/htmx.org@1.9.2" {}
            }
            body {
                h1 { "Machine de vote" }
                (vote_form(routes, lexicon))
                div hx-get=(routes.results) hx-trigger="every 3s" {
                    (voting_machine(routes, lexicon, machine))
                }
            }
        }
    }
}
