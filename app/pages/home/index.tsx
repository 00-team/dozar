import { Typing } from '!/components/typing'
import { Component, onMount } from 'solid-js'
import './style/home.scss'

const Home: Component<{}> = props => {
    let itemsWrapper: null | HTMLElement
    let titlesWrapper: null | HTMLElement

    onMount(() => {
        itemsWrapper = document.querySelector('.items-wrapper')
        titlesWrapper = document.querySelector('.titles-wrapper')
    })

    return (
        <main class='home'>
            <section class='hero-section'>
                <div class='titles-wrapper'>
                    <header class='hero-header section_title'>
                        <object data='/dozar.svg' type=''></object>
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
                                }, 500)
                            }}
                        />
                    </p>
                </div>

                <div class='items-wrapper'>
                    <HomeItem />
                    <HomeItem />
                </div>
                <div class='cta-wrapper'>
                    <button class='cta main '>اصلی</button>
                    <button class='cta '>فرعی</button>
                </div>
            </section>
        </main>
    )
}

export const HomeItem: Component = () => {
    return (
        <div class='hero-item'>
            <img src='https://picsum.photos/600/600' class='item-img' alt='' />
            <div class='item-detail'></div>
            <button class='item-goto button-basic title_small'>دیدن</button>
        </div>
    )
}

export default Home
