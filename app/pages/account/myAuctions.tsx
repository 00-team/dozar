import { lazy } from 'solid-js'
const AuctionCard = lazy(() => import('../auctions'))

import './style/myauctions.scss'

export const MyAuctions = () => {
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
