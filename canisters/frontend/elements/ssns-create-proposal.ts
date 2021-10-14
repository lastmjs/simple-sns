import {
    html,
    render as litRender,
    TemplateResult
} from 'lit-html';
import { createObjectStore } from 'reduxular';

type State = Readonly<{
    title: string;
    type: 'CANISTER_CREATE' | 'CANISTER_UPGRADE' | 'NOT_SET';
    url: string;
    wasmModule: Uint8Array;
}>;

const InitialState: State = {
    title: '',
    type: 'NOT_SET',
    url: '',
    wasmModule: new Uint8Array()
};

class SSNSCreateProposal extends HTMLElement {
    shadow = this.attachShadow({
        mode: 'closed'
    });
    store = createObjectStore(InitialState, (state: State) => litRender(this.render(state), this.shadow), this);

    render(state: State): TemplateResult {
        return html`
            <style>

            </style>

            <div>
                <div>
                    Title: <input type="text" @input=${(e: InputEvent) => this.store.title = (e.target as HTMLInputElement).value}>
                </div>

                <div>
                    Type:
                    <select @input=${(e: InputEvent) => this.store.type = (e.target as HTMLSelectElement).value as State['type']}>
                        <option value="CANISTER_CREATE">Create canister</option>
                        <option value="CANISTER_UPGRADE">Upgrade canister</option>
                    </select>
                </div>

                <div>
                    Url: <input type="text" @input=${(e: InputEvent) => this.store.url = (e.target as HTMLInputElement).value}>
                </div>

                <div>
                    Wasm module: <input type="file">
                </div>
            </div>
        `;
    }
}

window.customElements.define('ssns-create-proposal', SSNSCreateProposal);