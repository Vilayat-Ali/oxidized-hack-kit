"use client";

// lib
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import * as z from "zod";
import axios from "axios";

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

// Req
import { QueryMaker } from "@/utils/axios";

const Signup = () => {
  const { toast } = useToast();

  const SignUpSchema = z.object({
    first: z.string().min(1),
    last: z.string().min(1),
    email: z.string().email().min(1),
    password: z.string().min(8),
    confirmPassword: z.string().min(8),
  });

  const form = useForm<z.infer<typeof SignUpSchema>>({
    resolver: zodResolver(SignUpSchema),
    defaultValues: {
      first: "",
      last: "",
      email: "",
      password: "",
      confirmPassword: "",
    },
  });

  const onSubmit = async (values: z.infer<typeof SignUpSchema>) => {
    try {
      const { first, last, email, password } = values;
      const { data } = await axios.post(
        "http://localhost:8000/api/auth/register",
        {
          name: { first, last },
          email,
          password,
        }
      );
      console.log(data);
      window.localStorage.setItem("token", data.access_token);
      toast({
        title: "User added successfully!",
      });
    } catch (err: any) {
      toast({
        title: err.message,
      });
    }
  };

  const [showPassword, TogglePasswordVisibility] = useToggle();
  const [showConfirmPassword, ToggleConfirmPasswordVisibility] = useToggle();

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)}>
        <div className="flex flex-row justify-between items-center">
          <div className="w-[50%]">
            <FormField
              control={form.control}
              name="first"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>First Name</FormLabel>
                  <FormControl>
                    <Input placeholder="John" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
          <div className="w-[45%]">
            <FormField
              control={form.control}
              name="last"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Last Name</FormLabel>
                  <FormControl>
                    <Input placeholder="Doe" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
        </div>

        <div className="mt-2">
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

        <div className="mt-2">
          <FormField
            control={form.control}
            name="confirmPassword"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Confirm Password</FormLabel>
                <div className="flex flex-row items-center">
                  <FormControl>
                    <Input
                      type={showConfirmPassword ? "text" : "password"}
                      placeholder={showConfirmPassword ? "ABCDEF" : "XXXXX"}
                      {...field}
                    />
                  </FormControl>
                  <Button
                    type="button"
                    variant="outline"
                    onClick={ToggleConfirmPasswordVisibility}
                  >
                    {showConfirmPassword ? (
                      <MdVisibilityOff />
                    ) : (
                      <MdVisibility />
                    )}
                  </Button>
                </div>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>

        <Button type="submit" className="mt-5">
          Register
        </Button>
      </form>
    </Form>
  );
};

export default Signup;
