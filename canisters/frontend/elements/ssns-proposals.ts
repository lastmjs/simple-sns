import {
    html,
    render as litRender,
    TemplateResult
} from 'lit-html';
import { createObjectStore } from 'reduxular';
import { Proposal } from '../types/index.d';
import './ssns-proposal';

type State = Readonly<{
    proposals: ReadonlyArray<Proposal>;
}>;

const InitialState: State = {
    proposals: []
};

class SSNSProposals extends HTMLElement {
    shadow = this.attachShadow({
        mode: 'closed'
    });
    store = createObjectStore(InitialState, (state: State) => litRender(this.render(state), this.shadow), this);

    render(state: State): TemplateResult {
        return html`
            <style>
            </style>

            <div>
                ${state.proposals.map((proposal) => {
                    return html`
                        <div>
                            <ssns-proposal .proposal=${proposal}></ssns-proposal>
                        </div>
                    `;
                })}
            </div>
        `;
    }
}

window.customElements.define('ssns-proposals', SSNSProposals);