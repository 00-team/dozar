import { Component } from 'solid-js'
import './style/home.scss'

const Home: Component<{}> = props => {
    return (
        <main class='home'>
            <header class='home-header section_title'>
                <object data='/dozar.svg'></object>
                دوزار
            </header>
        </main>
    )
}

export default Home
