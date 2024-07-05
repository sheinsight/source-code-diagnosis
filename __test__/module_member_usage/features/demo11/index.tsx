import  * as antd from "antd"

function App(props) {

    useEffect(() => {
        let x = deepClone(props.user)
    },[])

    return (
        <div>
            <antd.Button />
        </div>
    )
}