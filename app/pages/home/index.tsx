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
            <img class='item-img' src='https://picsum.photos/300/300' alt='' />
            <div class='item-detail'>
                <h2 class='title_small'>لورم ایپسوم</h2>
            </div>
        </div>
    )
}

export default Home
