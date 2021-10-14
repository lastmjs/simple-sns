import {
    html,
    render as litRender,
    TemplateResult
} from 'lit-html';
import { createObjectStore } from 'reduxular';
import { Proposal } from '../types/index.d';
import './ssns-create-proposal';
import './ssns-proposals';
import './ssns-configuration';

type State = Readonly<{
    proposals: ReadonlyArray<Proposal>;
    showCreateProposal: boolean;
}>;

const InitialState: State = {
    proposals: [],
    showCreateProposal: false
};

class SSNSApp extends HTMLElement {
    shadow = this.attachShadow({
        mode: 'closed'
    });
    store = createObjectStore(InitialState, (state: State) => litRender(this.render(state), this.shadow), this);

    render(state: State): TemplateResult {
        return html`
            <style>
            </style>

            <div>
                <h1>Configuration</h1>

                <div>
                    <ssns-configuration></ssns-configuration>
                </div>

                <h1>Proposals</h1>

                <div>
                    <button
                        @click=${() => this.store.showCreateProposal = !this.store.showCreateProposal}
                    >
                        ${state.showCreateProposal == true ? 'Stop creating proposal' : 'Create proposal'}
                    </button>
                </div>

                <br>

                <div>
                    <ssns-create-proposal ?hidden=${!state.showCreateProposal}></ssns-create-proposal>
                </div>

                <br>

                <div>
                    <ssns-proposals .proposals=${state.proposals}></ssns-proposals>
                </div>
            </div>
        `;
    }
}

window.customElements.define('ssns-app', SSNSApp);