import {
    html,
    render as litRender,
    TemplateResult
} from 'lit-html';
import { createObjectStore } from 'reduxular';
import {
    Proposal
} from '../types/index.d';

type State = Readonly<{
    proposal: Proposal | 'NOT_SET';
}>;

const InitialState: State = {
    proposal: 'NOT_SET'
};

class SSNSProposal extends HTMLElement {
    shadow = this.attachShadow({
        mode: 'closed'
    });
    store = createObjectStore(InitialState, (state: State) => litRender(this.render(state), this.shadow), this);

    render(state: State): TemplateResult {
        if (state.proposal === 'NOT_SET') {
            return html`
                <div>Loading...</div>
            `;
        }

        return html`
            <style>

            </style>

            <div>
                <div>Title: ${state.proposal.title}</div>
    
                <div>
                    Url: <a href="${state.proposal.url}" target="_blank">${state.proposal.url}</a>
                </div>
                
                <div>
                    Wasm module: <button>Download</button>
                </div>
            </div>
        `;
    }
}

window.customElements.define('ssns-proposal', SSNSProposal);