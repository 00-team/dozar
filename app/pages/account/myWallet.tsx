import { Component } from 'solid-js'

import './style/mywallet.scss'

export const MyWallet: Component<{}> = props => {
    return (
        <section class='wallet-container'>
            <h2 class='title'>کیف پول من</h2>
            <div class='wallet-wrapper'>
                <div class='wallet-card'></div>
                <div class='wallet-input'></div>
                <button class='cta title'>واریز</button>
            </div>
        </section>
    )
}
