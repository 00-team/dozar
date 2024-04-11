import { Component } from 'solid-js'

import { AuctionCard } from '../auctions'
import './style/myauctionswin.scss'

export const MyAuctionWins: Component<{}> = props => {
    return (
        <div class='myauctionswin'>
            <h2 class='title'>مزایده های بنده شده</h2>
            <div class='auctions-wrapper'>
                <AuctionCard />
                <AuctionCard />
            </div>
        </div>
    )
}
