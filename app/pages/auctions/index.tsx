import { Component, createSignal } from 'solid-js'

import './style/auctions.scss'

const Auctions: Component<{}> = props => {
    const [filter, setFilter] = createSignal<'active' | 'inactive'>('active')
    return (
        <main class='auctions'>
            <div
                class='filter-wrapper'
                classList={{ active: filter() === 'active' }}
            >
                <button
                    class='filter title_smaller'
                    onclick={() => filter() !== 'active' && setFilter('active')}
                    classList={{ active: filter() === 'active' }}
                >
                    فعال
                </button>
                <button
                    class='filter title_smaller'
                    classList={{ active: filter() === 'inactive' }}
                    onclick={() =>
                        filter() !== 'inactive' && setFilter('inactive')
                    }
                >
                    غیر فعال
                </button>
            </div>
            <div class='auctions-wrapper'></div>
        </main>
    )
}

export default Auctions
