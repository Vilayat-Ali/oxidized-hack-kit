"use client";

// lib
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import * as z from "zod";

// components
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { useToast } from "@/components/ui/use-toast";
import { Input } from "@/components/ui/input";

// icons
import { MdVisibility, MdVisibilityOff } from "react-icons/md";

// hooks
import useToggle from "@/hooks/useToggle";

const Login = () => {
  const { toast } = useToast();

  const LoginSchema = z.object({
    email: z.string().email().min(1),
    password: z.string().min(8),
  });

  const form = useForm<z.infer<typeof LoginSchema>>({
    resolver: zodResolver(LoginSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  });

  const onSubmit = (values: z.infer<typeof LoginSchema>) => {
    // toast({
    //   title: "Hello",
    //   description: "World",
    // });
    console.log(values);
  };

  const [showPassword, TogglePasswordVisibility] = useToggle();

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)}>
        <div>
          <FormField
            control={form.control}
            name="email"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Email</FormLabel>
                <FormControl>
                  <Input placeholder="johndoe@email.com" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>

        <div className="mt-2">
          <FormField
            control={form.control}
            name="password"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Password</FormLabel>
                <div className="flex flex-row items-center">
                  <FormControl>
                    <Input
                      type={showPassword ? "text" : "password"}
                      placeholder={showPassword ? "ABCDEF" : "XXXXX"}
                      {...field}
                    />
                  </FormControl>
                  <Button
                    type="button"
                    variant="outline"
                    onClick={TogglePasswordVisibility}
                  >
                    {showPassword ? <MdVisibilityOff /> : <MdVisibility />}
                  </Button>
                </div>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>

        <Button type="submit" className="mt-5">
          Login
        </Button>
      </form>
    </Form>
  );
};

export default Login;
