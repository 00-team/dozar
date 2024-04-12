import { Component } from 'solid-js'

import './style/mywallet.scss'

import CountUp from '!/components/countUp'
import { PiggyBankIcon } from '!/icons/account'
import WalletImg from '!/static/imgs/wallet2.webp'
import { createStore } from 'solid-js/store'

export const MyWallet: Component<{}> = props => {
    const [wallet, setwallet] = createStore({
        holding: 1120405,
        deposit: 0,
    })

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
                        <p class='card-logo'>DOZAR</p>

                        <div class='card-chip'>
                            <div class='chip-line'></div>
                            <div class='chip-line'></div>
                            <div class='chip-line'></div>
                            <div class='chip-line'></div>
                            <div class='chip-main'></div>
                        </div>
                    </div>

                    <div class='card-main'>
                        <span class='holder title_smaller'>موجودی کیف پول</span>
                        <p class='title wallet-holding'>
                            <CountUp
                                format
                                end={wallet.holding}
                                steps={12345}
                                addTime={20}
                            />
                        </p>
                    </div>
                    <div class='card-bottom'></div>
                </div>
                <div class='wallet-input'>
                    <div class='holder title_smaller'>
                        <PiggyBankIcon />
                        واریز به کیف پول
                    </div>
                    <input
                        type='number'
                        class='title_smaller'
                        inputMode={'numeric'}
                        placeholder='مقدار واریزی...'
                        oninput={e => {
                            setwallet({ deposit: e.target.valueAsNumber })
                        }}
                        value={wallet.deposit}
                    />
                </div>
                <button class='cta title_smaller'>واریز</button>
            </div>
        </section>
    )
}
