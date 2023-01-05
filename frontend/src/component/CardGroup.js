import axios from 'axios'
import {useEffect, useState} from "react";

function CardGroupComponent() {

    const $list_group = document.getElementById('list-group');
    const $tag = document.getElementById('tag');

    let [posts, setPosts] = useState([])
    const [page, setPage] = useState(1)
    const [fetching, setFetching] = useState(true)
    let [total, setTotal] = useState(0);
    const [isTotal, setIsTotal] = useState(false);
    const [isFirstLoad, setIsFirstLoad] = useState(true);
    const [isTagLoaded, setIsTagLoaded] = useState(false);

    useEffect(() => {
        const isTotal = $list_group !== null && $list_group.children.length === total;

        if (fetching && !isTotal) {
            const tag = $tag !== null ? $tag.value : 'All';

            getPosts(page, tag)
                .then(response => {
                    setPosts([...posts, ...response.data.posts])
                    setPage(currentPage => currentPage + 1)
                    setTotal(response.data.total_count)
                })
                .finally(() => setFetching(false))
        }
    }, [fetching, isTotal]);

    useEffect(() => {
       if ($tag !== null && !isTagLoaded) {
           $tag.addEventListener('change' , tagHandler);
           setIsTagLoaded(true);
       }

    }, [$tag, isTagLoaded]);

    const tagHandler = () => {
        const tag = $tag.value;
        posts = []
        setPage(1);

        getPosts(1, tag)
            .then(response => {
                setPosts([...posts, ...response.data.posts])
                setPage(currentPage => currentPage + 1)

                setTotal(response.data.total_count)
            })
            .finally(() => setFetching(false))
            .finally(() => setIsTotal($list_group !== null && $list_group.children.length === total))
    }

    //todo если записей 0 то будет 2 запроса
    useEffect(() => {
        if ($list_group !== null) {

            if (isFirstLoad) {
                setIsFirstLoad(false);

                if (document.body.offsetHeight > document.documentElement.clientHeight) {
                    setFetching(false)
                } else {
                    setFetching(true);
                }
            }
        }

    }, [$list_group, isFirstLoad])

    useEffect(() => {
        document.addEventListener('scroll', scrollHandler)
    })

    const scrollHandler = (event) => {
        const scrollBorder =
            event.target.documentElement.scrollHeight - (event.target.documentElement.scrollTop + window.innerHeight);

        if (scrollBorder < 1) {
            setFetching(true);
        }
    }

    return (
        <div className="col">
            <div id="list-group">
                {posts.map(post =>
                    <div key={post.id} className="mb-3 list-group-item">
                        <div className="border-lite">
                            <a href={"/post/" + post.id}>
                                <div className="p-3 text-color">
                                    {post.published ? '' : <div className="border-red mb-2 ">Неопубликовано</div>}
                                    <div className="mb-3">{post.title}</div>
                                    <p className="">{post.preview}</p>
                                    <div className="">Дата публикаци: {post.public_date}</div>
                                </div>
                            </a>
                        </div>
                    </div>
                )}
            </div>
        </div>
    )
}

function getPosts(page, tag) {
    const token = localStorage.getItem("user-token");
    return  axios({
        method: 'get',
        url: 'http://localhost:8000/post/get',
        params: {
            page: page,
            tag: tag
        },
        headers: {
            'user-token': token
        }
    })
}

export default CardGroupComponent;
