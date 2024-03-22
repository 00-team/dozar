import { Typing } from '!/components/typing'
import { Component } from 'solid-js'
import './style/home.scss'

const Home: Component<{}> = props => {
    return (
        <main class='home'>
            <section class='hero-section'>
                <header class='hero-header section_title'>
                    <object data='/dozar.svg' type=''></object>
                    <p>دوزار</p>
                </header>
                <p class='title'>
                    {/* جایی برای کشف <span>گنجینه‌</span> های مخفی و{' '}
                    <span>خرید</span> های با ارزش */}
                    <Typing
                        speed={75}
                        sentence='جایی برای کشف گنجینه های مخفی و خرید های با ارزش'
                    />
                </p>
                <div class='items-wrapper'>
                    <HomeItem />
                    <HomeItem />
                </div>
                <div class='cta-wrapper'>
                    <button class='cta main'></button>
                    <button class='cta '></button>
                </div>
            </section>
        </main>
    )
}

export const HomeItem: Component = () => {
    return <div class='hero-item'></div>
}

export default Home
