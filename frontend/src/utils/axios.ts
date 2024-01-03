"use client";

// lib
import axios, { AxiosRequestConfig } from "axios";

// ENV
import { config } from "@/env";

export class QueryMaker {
  private endpoint: string = config.SERVER_URL;
  private JWT_Token: string = this.lookForJWT();

  private lookForJWT(): string {
    if (window) {
      const token: string | null = window.localStorage.getItem("JWT");

      if (typeof token === "string") {
        return `bearer ${token}`;
      }
    }
    return "";
  }

  public async Fetch<T>(route: string): Promise<T> {
    try {
      const { data } = await axios.get(this.endpoint.concat(route), {
        headers: {
          Authorization: this.JWT_Token,
        },
      });
      return data as T;
    } catch (err: any) {
      throw new Error(err);
    }
  }

  public async Mutate<P, R>(
    route: string,
    req_body: P & AxiosRequestConfig<P>,
    Method: "post" | "put" | "delete" = "post"
  ): Promise<R> {
    try {
      const { data } = await axios[Method](
        this.endpoint.concat(route),
        req_body,
        {
          headers: {
            Authorization: this.JWT_Token,
          },
        }
      );
      return data as R;
    } catch (err: any) {
      throw new Error(err);
    }
  }
}
