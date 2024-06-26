import ClockTick from '!/components/clockTick'
import { Typing } from '!/components/typing'
import { CalendarIcon, PriceIcon } from '!/icons/home'
import { Link } from '@solidjs/router'
import { Component, onMount } from 'solid-js'
import './style/home.scss'

const Home: Component<{}> = props => {
    let itemsWrapper: null | HTMLElement
    let titlesWrapper: null | HTMLElement
    let ctaWrapper: null | HTMLElement

    onMount(() => {
        itemsWrapper = document.querySelector('.items-wrapper')
        titlesWrapper = document.querySelector('.titles-wrapper')
        ctaWrapper = document.querySelector('.cta-wrapper')
    })

    return (
        <main class='home'>
            <section class='hero-section'>
                <div class='titles-wrapper'>
                    <header class='hero-header section_title'>
                        <object
                            data='/public/dozar.svg'
                            aria-labelledby='logo'
                            type=''
                        ></object>
                        <p>دوزار</p>
                    </header>
                    <p class='title'>
                        {/* جایی برای کشف <span>گنجینه‌</span> های مخفی و{' '}
                    <span>خرید</span> های با ارزش */}
                        <Typing
                            speed={65}
                            sentence='جایی برای کشف گنجینه های مخفی و خرید های با ارزش'
                            onFinish={() => {
                                setTimeout(() => {
                                    titlesWrapper.className += ' active'
                                    itemsWrapper.className += ' active'
                                    ctaWrapper.className += ' active'
                                }, 500)
                            }}
                        />
                    </p>
                </div>

                <div class='items-wrapper'>
                    <HomeItem />
                    <HomeItem />
                    <HomeItem />
                </div>
                <div class='cta-wrapper'>
                    <Link href='/auctions' class='cta main title_smaller'>
                        همه مزایده ها{' '}
                    </Link>
                </div>
            </section>
            <a
                referrerpolicy='origin'
                target='_blank'
                href='https://trustseal.enamad.ir/?id=496629&Code=duyzTreHs7iFitqyQtwSVTHxfqRh2bli'
            >
                <img
                    referrerpolicy='origin'
                    src='https://trustseal.enamad.ir/logo.aspx?id=496629&Code=duyzTreHs7iFitqyQtwSVTHxfqRh2bli'
                    alt=''
                    style='cursor:pointer'
                    code='duyzTreHs7iFitqyQtwSVTHxfqRh2bli'
                />
            </a>
        </main>
    )
}

export const HomeItem: Component = () => {
    return (
        <div class='hero-item'>
            <img
                src='https://picsum.photos/1200/600'
                class='item-img'
                loading='lazy'
                decoding='async'
                alt='dozar dozar-item'
            />
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
            <button class='item-goto button-basic title_small'>دیدن</button>
        </div>
    )
}

export default Home
