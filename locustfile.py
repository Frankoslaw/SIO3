from locust import HttpUser, task


class SIO3(HttpUser):
    @task
    def get_news(self):
        self.client.get("/c/1/news")
