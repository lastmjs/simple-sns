import {
    html,
    render as litRender,
    TemplateResult
} from 'lit-html';
import { createObjectStore } from 'reduxular';

type State = Readonly<{}>;

const InitialState: State = {};

class SSNSBlank extends HTMLElement {
    shadow = this.attachShadow({
        mode: 'closed'
    });
    store = createObjectStore(InitialState, (state: State) => litRender(this.render(state), this.shadow), this);

    render(state: State): TemplateResult {
        return html`
            <style>
            </style>

            <div>ssns-blank</div>
        `;
    }
}

window.customElements.define('ssns-blank', SSNSBlank);