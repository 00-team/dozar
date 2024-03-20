import { CalendarIcon, PriceIcon } from '!/icons/home'
import { Component } from 'solid-js'
import './style/home.scss'

const Home: Component<{}> = props => {
    return (
        <main class='home'>
            <header class='home-header'>
                <h1 class='section_title'>
                    <object data='/dozar.svg'></object>
                    دوزار
                </h1>
                <h3 class='title'>
                    جایی برای کشف گنجینه‌های مخفی و خریدهای باارزش
                </h3>
            </header>
            <div class='items-wrapper'>
                <Item />
                <Item />
                <Item />
                <Item />
                <Item />
                <Item />
                <Item />
                <Item />
                <Item />
            </div>
        </main>
    )
}

const Item: Component = () => {
    return (
        <div class='item'>
            <img class='item-img' src='https://picsum.photos/600/600' alt='' />
            <div class='item-wrapper'>
                <h2 class='title item-name'>لورم ایپسوم</h2>

                <div class='item-details'>
                    <div class='detail-row item-price '>
                        <div class='holder'>
                            <PriceIcon />
                            <p class='title_small'>قیمت پایه</p>
                        </div>
                        <div class='data number'>2,000</div>
                    </div>

                    <div class='detail-row item-init'>
                        <div class='holder'>
                            <CalendarIcon />
                            <p class='title_small'>زمان شروع </p>
                        </div>
                        <div class='data'>یسشیش</div>
                    </div>

                    <div class='detail-row item-clock'>
                        <div class='holder'>
                            <CalendarIcon />
                            <p class='title_small'>زمان باقی مانده</p>
                        </div>
                        <div class='data'>فلان قدر</div>
                    </div>
                </div>
            </div>
        </div>
    )
}

export default Home
