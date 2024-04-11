import { Component } from 'solid-js'

import './style/mywallet.scss'

import WalletImg from '!/static/imgs/wallet.webp'

export const MyWallet: Component<{}> = props => {
    return (
        <section class='wallet-container'>
            <h2 class='title'>کیف پول من</h2>
            <div class='wallet-wrapper'>
                <div class='wallet-card'>
                    <div class='card-cover'>
                        <img
                            src={WalletImg}
                            class='card-bg'
                            decoding={'async'}
                            loading='lazy'
                            alt=''
                        />
                    </div>
                    <div class='card-header'>
                        <div class='card-logo'></div>

                        <div class='card-chip'>
                            <div class='chip-line'></div>
                            <div class='chip-line'></div>
                            <div class='chip-line'></div>
                            <div class='chip-line'></div>
                            <div class='chip-main'></div>
                        </div>
                    </div>
                    <div class='card-main'>
                        <span class='holder'></span>
                        <p class='title wallet-holding'></p>
                    </div>
                    <div class='card-bottom'>
                        <span class='holder'></span>
                        <p class='username'></p>
                    </div>
                </div>
                <div class='wallet-input'></div>
                <button class='cta title'>واریز</button>
            </div>
        </section>
    )
}
