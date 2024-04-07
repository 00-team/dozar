import { Component } from 'solid-js'

import './style/auction.scss'

const Auction: Component<{}> = props => {
    return (
        <main class='auction'>
            <aside class='item-buy'>
                <img
                    src='https://picsum.photos/1920/1080'
                    alt=''
                    class='item-img'
                />
                <button class='cta title_small'>خرید</button>
                <p class='item-price title_small number'>
                    <span>192,100,000</span>
                </p>
            </aside>
            <aside class='item-details'></aside>
        </main>
    )
}

export default Auction
