import { WalletIcon } from '!/icons/account'
import { user } from '!/store/user'
import { useNavigate } from '@solidjs/router'
import { Component, onMount } from 'solid-js'

import './style/account.scss'

const Account: Component<{}> = props => {
    const navigate = useNavigate()

    onMount(() => {
        if (!user.token) return navigate('/login')
    })

    return (
        <main class='account-container'>
            <div class='account-wrapper'>
                <section class='account-header'>
                    <div class='wallet column'>
                        <WalletIcon />
                        <span class=' title_small'>کیف پول</span>
                    </div>
                    <div class='auctions column'>
                        <p class='title_small'>مزایده های شرکت کرده</p>
                        <span class='title_small number'>19</span>
                    </div>
                    <div class='auctions-win column'>
                        <p class='title_small'>مزایده های برنده شده </p>
                        <span class='title_small number'>19</span>
                    </div>
                    <div class='user_name column'>
                        <p class='title_small'>صدرا ادلاعی پور</p>
                        <button class='title_smaller'>ویرایش پروفایل</button>
                    </div>
                    <div class='user_img'>
                        <img src='https://picsum.photos/300/300' alt='' />
                    </div>
                </section>
                <section class='account-body'></section>
            </div>
        </main>
    )
}

export default Account
