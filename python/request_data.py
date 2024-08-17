class RequestData:
    def __init__(
        self,
        url,
        path=None,
        method="GET",
        headers=None,
        body=None,
        query=None,
    ):
        self.url = url
        self.path = path
        self.method = method.upper()
        self.headers = headers or {}
        self.body = body or {}
        self.query = query or {}

    def __str__(self):
        return f"RequestData(url={self.url}, paht={self.path},method={self.method}, headers={self.headers}, body={self.body}, query={self.query})"
