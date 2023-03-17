import Sql from "../components/Sql";
import Rest from "../components/Rest";
import GraphQL from "../components/Graphql";
import Schema from "../components/Schema";

export default [
  {
    path: "/",
    Component: Schema,
  },
  {
    path: "/rest",
    Component: Rest,
  },
  {
    path: "/sql",
    Component: Sql,
  },
  {
    path: "/graphql",
    Component: GraphQL,
  },
];
