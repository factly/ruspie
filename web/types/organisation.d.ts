export interface Organisation {
  id: string;
  createdAt: string;
  updatedAt: string;
  title: string;
  logo?: string;
  description?: string;
  projects?: Project[];
}

export interface Project {
  id: string;
  createdAt: string;
  updatedAt: string;
  title: string;
  description?: string;
}
