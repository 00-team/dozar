import { Component } from 'solid-js'
import { AuctionCard } from '../auctions'

import './style/myauctions.scss'

export const MyAuctions: Component<{}> = props => {
    return (
        <div class='myauctions'>
            <h2 class='title'>مزایده های من</h2>
            <div class='auctions-wrapper'>
                <AuctionCard />
                <AuctionCard />
                <AuctionCard />
                <AuctionCard />
                <AuctionCard />
            </div>
        </div>
    )
}
