import ClockTick from '!/components/clockTick'
import { CalendarIcon, PriceIcon } from '!/icons/home'
import { AuctionModel } from '!/store/auction'
import { Link } from '@solidjs/router'
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

export const AuctionCard: Component = () => {
    return (
        <div class='item'>
            <div class='item-img-wrapper'>
                <img
                    class='item-img'
                    src='https://picsum.photos/600/600'
                    alt=''
                />
            </div>
            <div class='item-wrapper'>
                <h2 class='title item-name'>لورم ایپسوم</h2>

                <div class='item-details'>
                    <div class='detail-row item-price '>
                        <div class='holder'>
                            <PriceIcon />
                            <p class='title_smaller'>قیمت پایه</p>
                        </div>
                        <div class='data number'>2,000</div>
                    </div>

                    <div class='detail-row item-init'>
                        <div class='holder'>
                            <CalendarIcon />
                            <p class='title_smaller'>زمان شروع </p>
                        </div>
                        <div class='data'>یسشیش</div>
                    </div>

                    <div class='detail-row item-clock'>
                        <div class='holder'>
                            <ClockTick />
                            <p class='title_smaller'>زمان باقی مانده</p>
                        </div>
                        <div class='data'>فلان قدر</div>
                    </div>
                </div>
            </div>
            <Link class='title_small goto' href='/auction/1'>
                دیدن
            </Link>
        </div>
    )
}

export default Auctions
