import { Component } from 'solid-js'

import './style/auction.scss'

const Auction: Component<{}> = props => {
    return (
        <main class='auction'>
            <aside class='item-buy'>
                <img
                    src='https://picsum.photos/1920/1080'
                    alt=''
                    class='item-img'
                />
                <button class='cta title_small'>خرید</button>
                <p class='item-price title_small number'>
                    <span>192,100,000</span>
                </p>
            </aside>
            <aside class='item-details'>
                <div class='item-intro'>
                    <h2 class='title item-title'>لورم ایپسوم ایز ده</h2>
                    <p class='title_smaller'>
                        لورم ایپسوم متن ساختگی با تولید سادگی نامفهوم از صنعت
                        چاپ و با استفاده از طراحان گرافیک است چاپگرها و متون هدف
                        بهبود ابزارهای کاربردی می باشد کتابهای زیادی در شصت و سه
                        درصد گذشته حال و آینده چاپ و با استفاده از طراحان گرافیک
                        است چاپگرها و متون هدف بهبود ابزارهای کاربردی می باشد
                        کتابهای زیادی در شصت و سه درصد گذشته حال و آینده
                    </p>
                </div>
                <div class='item-buyers'>
                    <h3 class='title'>
                        <span>لیست خریداران</span>
                    </h3>
                    <div class='buyers-wrapper'></div>
                </div>
            </aside>
        </main>
    )
}

export default Auction
