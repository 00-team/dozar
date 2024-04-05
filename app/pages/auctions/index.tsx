import { AuctionModel } from '!/store/auction'
import { Component, createSignal } from 'solid-js'

import './style/auctions.scss'

const Auctions: Component<{}> = props => {
    const [filter, setFilter] = createSignal<'active' | 'inactive'>('active')
    return (
        <main class='auctions'>
            <header
                class='filter-wrapper'
                classList={{ active: filter() === 'active' }}
            >
                <button
                    class='filter title_smaller'
                    onclick={() => filter() !== 'active' && setFilter('active')}
                    classList={{ active: filter() === 'active' }}
                >
                    فعال
                </button>
                <button
                    class='filter title_smaller'
                    classList={{ active: filter() === 'inactive' }}
                    onclick={() =>
                        filter() !== 'inactive' && setFilter('inactive')
                    }
                >
                    غیر فعال
                </button>
            </header>
            <div class='auctions-wrapper'>
                <AuctionCard />
                <AuctionCard />
                <AuctionCard />
                <AuctionCard />
                <AuctionCard />
                <AuctionCard />
            </div>
        </main>
    )
}

interface AuctionCardProps extends AuctionModel {}

const AuctionCard: Component = () => {
    return <div class='auction-card'></div>
}

export default Auctions
