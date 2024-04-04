import { AuctionIcon, ContactusIcon, HomeIcon, MenuIcon } from '!/icons/navbar'
import { Link } from '@solidjs/router'
import { Component, createSignal, JSX } from 'solid-js'

import './style/navbar.scss'

const Navbar: Component<{}> = props => {
    return (
        <>
            <NavPc />
            <NavMobile />
        </>
    )
}

const NavMobile: Component = () => {
    const [openNav, setOpenNav] = createSignal(false)
    return (
        <>
            <nav class='nav-container-mobile'>
                <div class='nav-open' onclick={() => setOpenNav(true)}>
                    <MenuIcon size={35} />
                </div>
                <Link href='/' class='nav-logo hero_title'>
                    DOZAR
                </Link>
            </nav>
            <div
                class='nav-mobile-open'
                classList={{ active: openNav() }}
            ></div>
        </>
    )
}

const NavPc = () => {
    return (
        <nav class='nav-container-pc'>
            <div class='nav-links'>
                <NavLinkPC Icon={HomeIcon} link='/' title='صفحه اصلی' />
                <NavLinkPC Icon={AuctionIcon} link='/' title='مزایده ها' />
                <NavLinkPC Icon={ContactusIcon} link='/' title='ارتباط با ما' />
            </div>
            <Link href='/' class='nav-logo hero_title'>
                DOZAR
            </Link>
        </nav>
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
