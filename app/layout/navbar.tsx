import { Component } from 'solid-js'

import './style/navbar.scss'

const Navbar: Component<{}> = props => {
    return (
        <nav class='nav-container'>
            <div class='nav-logo title'>DOZAR</div>
            <p class='nav-title'></p>
            <div class='nav-open'></div>
        </nav>
    )
}

export default Navbar
