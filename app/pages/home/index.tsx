import ClockTick from '!/components/clockTick'
import { CalendarIcon, PriceIcon } from '!/icons/home'
import { Component, onMount } from 'solid-js'
import './style/home.scss'

const Home: Component<{}> = props => {
    let scrollElems: null | NodeListOf<Element>

    onMount(() => {
        scrollElems = document.querySelectorAll('span.shadow-scroll')
    })

    return (
        <main
            class='home'
            onscroll={e => {
                let y = e.target.scrollTop

                if (y > 500) return

                scrollElems.forEach((elem: HTMLElement) => {
                    elem.style.filter = `drop-shadow(1px ${Math.max(15 - y * 0.2, -15)}px ${Math.max(4 - y * 0.05, 1)}px rgb(0, 0, 0, 0.5))`
                })
            }}
        >
            <header class='home-header'>
                <h1 class='section_title'>
                    <object data='/dozar.svg'></object>
                    دوزار
                </h1>
                <h3 class='title'>
                    جایی برای کشف <span class='shadow-scroll'>گنجینه‌</span> های
                    مخفی و <span class='shadow-scroll'>خرید</span> های با ارزش
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
                            <ClockTick />
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
