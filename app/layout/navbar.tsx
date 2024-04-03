import { AuctionIcon, ContactusIcon, HomeIcon } from '!/icons/navbar'
import { Link } from '@solidjs/router'
import { Component, JSX } from 'solid-js'

import './style/navbar.scss'

const Navbar: Component<{}> = props => {
    let isMobile = innerWidth <= 768
    return (
        <>
            {isMobile ? (
                <></>
            ) : (
                <nav class='nav-container'>
                    <div class='nav-links'>
                        <NavLinkPC Icon={HomeIcon} link='/' title='صفحه اصلی' />
                        <NavLinkPC
                            Icon={AuctionIcon}
                            link='/'
                            title='مزایده ها'
                        />
                        <NavLinkPC
                            Icon={ContactusIcon}
                            link='/'
                            title='ارتباط با ما'
                        />
                    </div>
                    <Link href='/' class='nav-logo hero_title'>
                        DOZAR
                    </Link>
                </nav>
            )}
        </>
    )
}

interface NavLinkPc {
    Icon: (P: any) => JSX.Element | JSX.Element
    title: string
    link: string
}
const NavLinkPC: Component<NavLinkPc> = ({ Icon, link, title }) => {
    return (
        <Link href={link} class='nav-link title_small'>
            <div class='holder icon'>
                <Icon />
            </div>
            <p class='data'>{title}</p>
        </Link>
    )
}

export default Navbar
