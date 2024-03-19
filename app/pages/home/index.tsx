import { Component } from 'solid-js'
import './style/home.scss'

const Home: Component<{}> = props => {
    return (
        <main class='home'>
            <header class='home-header section_title'>
                <object data='/dozar.svg'></object>
                دوزار
            </header>
            <div class='items-wrapper'>
                <div class='item'></div>
                <div class='item'></div>
                <div class='item'></div>
            </div>
        </main>
    )
}

export default Home
