import { Button, Card, Flex, Form, Input } from "antd";
import useNotification from "antd/es/notification/useNotification";
import { useNavigate } from "react-router-dom";
import { LoginUseCase } from "../../../app/use-cases/login-use-case";

export default function LoginPage() {

  const [notification, context] = useNotification();
  const navigator = useNavigate();

  const handleFinish = (fields: any) => {
    const useCase = new LoginUseCase(navigator, notification);
    useCase.execute(fields.username, fields.password);
  }

  return (
    <Flex justify="center" align="center" style={{ height: "100%" }}>
      {context}
      <Card title="Login">
        <Form onFinish={handleFinish}>
          <Form.Item name="username">
            <Input placeholder="username" />
          </Form.Item>

          <Form.Item name="password">
            <Input type="password" placeholder="password" />
          </Form.Item>

          <Button htmlType="submit" type="primary">
            Login
          </Button>
        </Form>
      </Card>
    </Flex>
  )
}
